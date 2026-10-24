#include <algorithm>
#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2>
void f51(std::forward_list<T1, T2> &a0, const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2> void f52(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
void f53(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(std::move(a1));
}

template <typename T1, typename T2, typename T3>
void f54(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(std::move(a1));
}

template <typename T1, typename T2>
bool f55(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f56(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f57(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f58(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f59(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
