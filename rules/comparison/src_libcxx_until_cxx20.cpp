#include "../common/init.h"
#include "../common/traits.h"
#include <queue>
#include <utility>
#include <vector>

template <typename T1, typename T2>
bool f1(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f2(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f3(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f4(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f5(const std::vector<T1, T2> &a0, const std::vector<T1, T2> &a1) {
  return a0 >= a1;
}
