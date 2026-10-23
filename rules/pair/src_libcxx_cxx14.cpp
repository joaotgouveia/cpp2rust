#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
const T1 &&f63(const std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
const T1 &&f64(const std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f65(std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f66(std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> const T1 &f67(const std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> const T1 &f68(const std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f69(std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f70(std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}
