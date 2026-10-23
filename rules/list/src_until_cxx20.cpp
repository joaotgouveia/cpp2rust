#include <algorithm>
#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
void f53(std::list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2> void f54(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
void f55(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f56(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}
