#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
bool f44(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) != a1;
}

template <typename T1>
bool f45(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 != std::move(a1);
}

template <typename T1>
bool f46(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) < a1;
}

template <typename T1>
bool f47(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 < std::move(a1);
}

template <typename T1>
bool f48(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) <= a1;
}

template <typename T1>
bool f49(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 <= std::move(a1);
}

template <typename T1>
bool f50(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) == a1;
}

template <typename T1>
bool f51(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) > a1;
}

template <typename T1>
bool f52(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 > std::move(a1);
}

template <typename T1>
bool f53(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) >= a1;
}

template <typename T1>
bool f54(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 >= std::move(a1);
}
