#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2>
const T1 &&f74(const std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
const T1 &&f75(const std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f76(std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f77(std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> const T1 &f78(const std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> const T1 &f79(const std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f80(std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f81(std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}
