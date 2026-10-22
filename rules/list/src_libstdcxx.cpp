#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f78(std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f79(const std::list<T1, T2> &a0) {
  return a0.front();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::reference f80(std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
typename std::list<T1, T2>::const_reference f81(const std::list<T1, T2> &a0) {
  return a0.back();
}

template <typename T1, typename T2>
void f82(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}
