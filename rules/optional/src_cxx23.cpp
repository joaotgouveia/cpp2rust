#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> auto f33(std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f34(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f35(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f36(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template and_then<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f37(std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f38(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2> auto f39(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
auto f40(const std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template transform<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f41(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template or_else<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> f42(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template or_else<T2>(std::forward<T2>(a1));
}
