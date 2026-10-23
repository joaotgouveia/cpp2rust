#include <algorithm>
#include <utility>

template <typename T1, typename T2>
const T1 &&f18(const std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
const T1 &&f19(const std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f20(std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f21(std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> const T1 &f22(const std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> const T1 &f23(const std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f24(std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f25(std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}
