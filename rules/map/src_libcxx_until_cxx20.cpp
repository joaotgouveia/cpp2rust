#include <initializer_list>
#include <map>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4>
bool f93(const std::map<T1, T2, T3, T4> &a0,
         const std::map<T1, T2, T3, T4> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f94(const std::multimap<T1, T2, T3, T4> &a0,
         const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 != a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f95(const std::map<T1, T2, T3, T4> &a0,
         const std::map<T1, T2, T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f96(const std::multimap<T1, T2, T3, T4> &a0,
         const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 < a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f97(const std::map<T1, T2, T3, T4> &a0,
         const std::map<T1, T2, T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f98(const std::multimap<T1, T2, T3, T4> &a0,
         const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 <= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f99(const std::map<T1, T2, T3, T4> &a0,
         const std::map<T1, T2, T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f100(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 > a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f101(const std::map<T1, T2, T3, T4> &a0,
          const std::map<T1, T2, T3, T4> &a1) {
  return a0 >= a1;
}

template <typename T1, typename T2, typename T3, typename T4>
bool f102(const std::multimap<T1, T2, T3, T4> &a0,
          const std::multimap<T1, T2, T3, T4> &a1) {
  return a0 >= a1;
}
