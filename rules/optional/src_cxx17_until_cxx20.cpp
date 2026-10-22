#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
bool f13(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) != a1;
}

template <typename T1>
bool f14(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 != std::move(a1);
}

template <typename T1>
bool f15(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) < a1;
}

template <typename T1>
bool f16(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 < std::move(a1);
}

template <typename T1>
bool f17(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) <= a1;
}

template <typename T1>
bool f18(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 <= std::move(a1);
}

template <typename T1>
bool f19(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) == a1;
}

template <typename T1>
bool f20(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) > a1;
}

template <typename T1>
bool f21(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 > std::move(a1);
}

template <typename T1>
bool f22(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::move(a0) >= a1;
}

template <typename T1>
bool f23(const std::optional<T1> &a0, std::nullopt_t a1) {
  return a0 >= std::move(a1);
}
