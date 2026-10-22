#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f36(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f37(const std::tuple<T3, T4> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f38(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> f39(std::tuple<T3, T4> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f40(std::pair<T1, T2> &a0, const std::tuple<T3, T4> &a1) {
  return a0.template operator= <T3, T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
std::pair<T1, T2> &f41(std::pair<T1, T2> &a0, std::tuple<T3, T4> &&a1) {
  return a0.template operator= <T3, T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f42(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f43(const std::array<T3, 2> &a0) {
  return std::pair<T1, T2>(a0);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f44(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> f45(std::array<T3, 2> &&a0) {
  return std::pair<T1, T2>(std::move(a0));
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f46(std::pair<T1, T2> &a0, const std::array<T3, 2> &a1) {
  return a0.template operator= <T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::pair<T1, T2> &f47(std::pair<T1, T2> &a0, std::array<T3, 2> &&a1) {
  return a0.template operator= <T3>(std::move(a1));
}
