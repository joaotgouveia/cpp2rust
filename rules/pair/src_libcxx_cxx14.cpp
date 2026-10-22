#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2>
const T1 &&f48(const std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2>
const T1 &&f49(const std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f50(std::pair<T1, T2> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> T1 &&f51(std::pair<T2, T1> &&a0) {
  return std::get<T1, T2>(std::move(a0));
}

template <typename T1, typename T2> const T1 &f52(const std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> const T1 &f53(const std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f54(std::pair<T1, T2> &a0) {
  return std::get<T1, T2>(a0);
}

template <typename T1, typename T2> T1 &f55(std::pair<T2, T1> &a0) {
  return std::get<T1, T2>(a0);
}
