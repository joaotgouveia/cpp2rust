#include "../common/init.h"
#include "../common/traits.h"
#include <array>
#include <tuple>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
bool f26(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f27(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f28(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f29(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f30(const std::pair<T1, T2> &a0, const std::pair<T3, T4> &a1) {
  return a0 >= a1;
}
