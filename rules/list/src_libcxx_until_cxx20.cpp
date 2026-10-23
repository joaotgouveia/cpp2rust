#include <initializer_list>
#include <list>
#include <utility>

template <typename T1, typename T2>
bool f87(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator!= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
void f88(std::list<T1, T2> &a0,
         const typename std::list<T1, T2>::value_type &a1) {
  return a0.remove(a1);
}

template <typename T1, typename T2>
bool f89(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator< <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f90(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator<= <T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f91(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator><T1, T2>(a0, a1);
}

template <typename T1, typename T2>
bool f92(const std::list<T1, T2> &a0, const std::list<T1, T2> &a1) {
  return std::operator>= <T1, T2>(a0, a1);
}
