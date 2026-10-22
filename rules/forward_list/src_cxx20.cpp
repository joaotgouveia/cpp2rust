#include "../common/init.h"
#include "../common/traits.h"
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f53(std::forward_list<T1, T2> &a0,
                                                  const T3 &a1) {
  return std::erase<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f54(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return std::erase_if<T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f55(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f56(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f57(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2>
auto f58(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return a0 <=> a1;
}
