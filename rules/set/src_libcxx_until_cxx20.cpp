#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3>
bool f76(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f77(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3>
bool f78(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f79(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3>
bool f80(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f81(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3>
bool f82(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f83(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3>
bool f84(const std::multiset<T1, T2, T3> &a0,
         const std::multiset<T1, T2, T3> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3>
bool f85(const std::set<T1, T2, T3> &a0, const std::set<T1, T2, T3> &a1) {
  return a0 >= a1;
}
