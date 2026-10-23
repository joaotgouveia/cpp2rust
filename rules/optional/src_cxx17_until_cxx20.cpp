#include <algorithm>
#include <initializer_list>
#include <optional>
#include <type_traits>
#include <utility>

template <typename T1>
bool f19(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator!= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f20(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator!= <T1>(a0, std::move(a1));
}

template <typename T1>
bool f21(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator< <T1>(std::move(a0), a1);
}

template <typename T1>
bool f22(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator< <T1>(a0, std::move(a1));
}

template <typename T1>
bool f23(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator<= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f24(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator<= <T1>(a0, std::move(a1));
}

template <typename T1>
bool f25(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator== <T1>(std::move(a0), a1);
}

template <typename T1>
bool f26(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator><T1>(std::move(a0), a1);
}

template <typename T1>
bool f27(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator><T1>(a0, std::move(a1));
}

template <typename T1>
bool f28(std::nullopt_t a0, const std::optional<T1> &a1) {
  return std::operator>= <T1>(std::move(a0), a1);
}

template <typename T1>
bool f29(const std::optional<T1> &a0, std::nullopt_t a1) {
  return std::operator>= <T1>(a0, std::move(a1));
}
