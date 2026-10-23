#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
bool f18(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator!= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f19(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator!= <T1>(a0, std::move(a1));
}

template <typename T1>
bool f20(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator< <T1>(std::move(a0), a1);
}

template <typename T1>
bool f21(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator< <T1>(a0, std::move(a1));
}

template <typename T1>
bool f22(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator<= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f23(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator<= <T1>(a0, std::move(a1));
}

template <typename T1>
bool f24(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator== <T1>(std::move(a0), a1);
}

template <typename T1>
bool f25(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator><T1>(std::move(a0), a1);
}

template <typename T1>
bool f26(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator><T1>(a0, std::move(a1));
}

template <typename T1>
bool f27(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator>= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f28(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator>= <T1>(a0, std::move(a1));
}
