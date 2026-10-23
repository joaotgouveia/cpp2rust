#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

using t4 = std::bad_optional_access;

std::bad_optional_access f43() { return std::bad_optional_access(); }

const char *f44(const std::bad_optional_access &a0) { return a0.what(); }

std::bad_optional_access f45(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f46(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

template <typename T1>
std::enable_if_t<std::is_constructible_v<std::decay_t<T1>, T1>,
                 std::optional<std::decay_t<T1>>>
f47(T1 &&a0) {
  return std::make_optional<T1>(std::forward<T1>(a0));
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, std::optional<T1>>
f48(T2 &&...a0) {
  return std::make_optional<T1, T2...>(std::forward<T2>(a0)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>,
    std::optional<T1>>
f49(std::initializer_list<T2> a0, T3 &&...a1) {
  return std::make_optional<T1, T2, T3...>(a0, std::forward<T3>(a1)...);
}

template <typename T1> const T1 *f50(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f51(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> const T1 &f52(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> const T1 &&f53(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> const T1 &f54(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> const T1 &&f55(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename... T2>
std::enable_if_t<std::is_constructible_v<T1, T2...>, T1 &>
f56(std::optional<T1> &a0, T2 &&...a1) {
  return a0.template emplace<T2...>(std::forward<T2>(a1)...);
}

template <typename T1, typename T2, typename... T3>
std::enable_if_t<
    std::is_constructible_v<T1, std::initializer_list<T2> &, T3...>, T1 &>
f57(std::optional<T1> &a0, std::initializer_list<T2> a1, T3 &&...a2) {
  return a0.template emplace<T2, T3...>(a1, std::forward<T3>(a2)...);
}

template <typename T1>
std::enable_if_t<std::is_move_constructible_v<T1> && std::is_swappable_v<T1>>
f58(std::optional<T1> &a0, std::optional<T1> &a1) {
  return std::swap<T1>(a0, a1);
}
