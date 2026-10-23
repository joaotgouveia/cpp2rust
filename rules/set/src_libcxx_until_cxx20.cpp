#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f85(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f86(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator!= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f87(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f88(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator< <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f89(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f90(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator<= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f91(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f92(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator><T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f93(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}

template <typename T1, typename T2, typename T3>
bool f94(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return std::operator>= <T1, T2, T3>(a0, a1);
}
