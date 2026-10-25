#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

using t1 = std::nullopt_t;

std::nullopt_t f1(const std::nullopt_t &a0) { return std::nullopt_t(a0); }

std::nullopt_t f2(std::nullopt_t &&a0) { return std::nullopt_t(std::move(a0)); }

std::nullopt_t &f3(std::nullopt_t &a0, const std::nullopt_t &a1) {
  return a0.operator=(a1);
}

std::nullopt_t &f4(std::nullopt_t &a0, std::nullopt_t &&a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> bool f5(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator== <T1>(a0, std::move(a1));
}

template <typename T1> using t2 = std::optional<T1>;

template <typename T1> using t3 = typename std::optional<T1>::value_type;

template <typename T1> std::optional<T1> f6() { return std::optional<T1>(); }

template <typename T1> std::optional<T1> f7(std::nullopt_t a0) {
  return std::optional<T1>(std::move(a0));
}

template <typename T1>
std::optional<T1> &f8(std::optional<T1> &a0, std::nullopt_t a1) {
  return a0.operator=(std::move(a1));
}

template <typename T1> void f9(std::optional<T1> &a0, std::optional<T1> &a1) {
  return a0.swap(a1);
}

template <typename T1> T1 &f10(std::optional<T1> &a0) { return a0.operator*(); }

template <typename T1> T1 &&f11(std::optional<T1> &&a0) {
  return std::move(a0).operator*();
}

template <typename T1> bool f12(const std::optional<T1> &a0) {
  return static_cast<bool>(a0);
}

template <typename T1> bool f13(const std::optional<T1> &a0) {
  return a0.has_value();
}

template <typename T1> T1 &f14(std::optional<T1> &a0) { return a0.value(); }

template <typename T1> T1 &&f15(std::optional<T1> &&a0) {
  return std::move(a0).value();
}

template <typename T1, typename T2 = std::remove_cv_t<T1>>
T1 f16(const std::optional<T1> &a0, T2 &&a1) {
  return a0.template value_or<T2>(std::forward<T2>(a1));
}

template <typename T1, typename T2 = std::remove_cv_t<T1>>
T1 f17(std::optional<T1> &&a0, T2 &&a1) {
  return std::move(a0).template value_or<T2>(std::forward<T2>(a1));
}
