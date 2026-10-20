#include "../common/init.h"
#include "../common/traits.h"
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
bool f66(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2>
bool f67(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2>
bool f68(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2>
bool f69(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2>
bool f70(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 >= a1;
}
