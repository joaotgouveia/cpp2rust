#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f115(std::list<T1, T2> &a0,
                                           const T1 &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
typename std::list<T1, T2>::size_type f116(std::list<T1, T2> &a0) {
  return a0.unique();
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f117(std::list<T1, T2> &a0, T3 a1) {
  return a0.template remove_if<T3>(a1);
}

template <typename T1, typename T2, typename T3>
typename std::list<T1, T2>::size_type f118(std::list<T1, T2> &a0, T3 a1) {
  return a0.template unique<T3>(a1);
}

template <typename T1, typename T2>
auto f119(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator<=> <T1, T2>(a0, a1);
}
