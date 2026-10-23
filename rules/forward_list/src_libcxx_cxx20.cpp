#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type f92(std::forward_list<T1, T2> &a0,
                                                  const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
typename std::forward_list<T1, T2>::size_type
f93(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f94(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::forward_list<T1, T2>::size_type f95(std::forward_list<T1, T2> &a0,
                                                  T3 a1) {
  return a0.template unique<T3>(a1);
}
