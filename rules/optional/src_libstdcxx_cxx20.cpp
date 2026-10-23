#include <algorithm>
#include <compare>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1, typename T2> std::optional<T1> f67(T2 &&a0) {
  return std::optional<T1>(std::forward<T2>(a0));
}

template <typename T1, typename T2>
std::optional<T1> f68(const std::optional<T2> &a0) {
  return std::optional<T1>(a0);
}

template <typename T1, typename T2>
std::optional<T1> f69(std::optional<T2> &&a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1, typename... T2>
std::optional<T1> f70(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::optional<T1> f71(std::in_place_t a0, std::initializer_list<T2> a1,
                      T3 &&...a2) {
  return std::optional<T1>(std::move(a0), a1, std::forward<T3>(a2)...);
}

template <typename T1, typename T2>
std::optional<T1> &f72(std::optional<T1> &a0, T2 &&a1) {
  return a0.template operator= <T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2>
std::optional<T1> &f73(std::optional<T1> &a0, const std::optional<T2> &a1) {
  return a0.template operator= <T2>(a1);
}

template <typename T1, typename T2>
std::optional<T1> &f74(std::optional<T1> &a0, std::optional<T2> &&a1) {
  return a0.template operator= <T2>(std::move(a1));
}
