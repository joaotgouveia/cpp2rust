#include <deque>
#include <initializer_list>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2>
bool f71(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f72(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f73(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f74(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f75(const std::deque<T1, T2> &a0, const std::deque<T1, T2> &a1) {
  return a0 >= a1;
}
