#include "../common/init.h"
#include "../common/traits.h"
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f49(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f50(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f51(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f52(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f53(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 >= a1;
}
