#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
bool f61(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) != a1;
}

template <typename T1>
bool f62(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 != std::move(a1);
}

template <typename T1>
bool f63(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) < a1;
}

template <typename T1>
bool f64(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 < std::move(a1);
}

template <typename T1>
bool f65(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) <= a1;
}

template <typename T1>
bool f66(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 <= std::move(a1);
}

template <typename T1>
bool f67(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) == a1;
}

template <typename T1>
bool f68(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) > a1;
}

template <typename T1>
bool f69(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 > std::move(a1);
}

template <typename T1>
bool f70(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) >= a1;
}

template <typename T1>
bool f71(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 >= std::move(a1);
}
