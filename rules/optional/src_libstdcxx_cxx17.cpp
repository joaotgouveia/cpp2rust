#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

using t4 = std::bad_optional_access;

std::bad_optional_access f68() { return std::bad_optional_access(); }

const char *f69(const std::bad_optional_access &a0) { return a0.what(); }

std::bad_optional_access f70(const std::bad_optional_access &a0) {
  return std::bad_optional_access(a0);
}

std::bad_optional_access &f71(std::bad_optional_access &a0,
                              const std::bad_optional_access &a1) {
  return a0.operator=(a1);
}

template <typename T1> const T1 *f72(const std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> T1 *f73(std::optional<T1> &a0) {
  return a0.operator->();
}

template <typename T1> const T1 &f74(const std::optional<T1> &a0) {
  return a0.operator*();
}

template <typename T1> T1 &f75(std::optional<T1> &a0) { return a0.operator*(); }

template <typename T1> T1 &&f76(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> const T1 &&f77(const std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> bool f78(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1> const T1 &f79(const std::optional<T1> &a0) {
  return a0.value();
}

template <typename T1> T1 &f80(std::optional<T1> &a0) { return a0.value(); }

template <typename T1> T1 &&f81(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> const T1 &&f82(const std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1> void f83(std::optional<T1> &a0) { return a0.reset(); }

template <typename T1, typename... T2>
std::optional<T1> f84(std::in_place_t a0, T2 &&...a1) {
  return std::optional<T1>(std::move(a0), std::forward<T2>(a1)...);
}
