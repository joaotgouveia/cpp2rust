#include <forward_list>
#include <initializer_list>
#include <utility>

template <typename T1, typename T2> void f43(std::forward_list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
void f44(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
void f45(std::forward_list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2>
bool f46(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f47(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f48(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f49(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f50(const std::forward_list<T1, T2> &a0,
         const std::forward_list<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
