#include "../common/init.h"
#include "../common/traits.h"
#include <deque>
#include <utility>

template <typename T1, typename T2>
bool f3(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f4(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f5(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f6(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f7(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 >= a1;
}
