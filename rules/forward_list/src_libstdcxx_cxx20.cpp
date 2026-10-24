#include <algorithm>
#include <forward_list>
#include <utility>

template <typename T1, typename T2>
std::size_t f75(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
std::size_t f76(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
std::size_t f77(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
std::size_t f78(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}
