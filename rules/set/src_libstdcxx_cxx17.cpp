#include "../common/init.h"
#include "../common/traits.h"
#include <algorithm>
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
void f69(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f70(std::set<T1, T2, T3> &a0, std::set<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f71(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f72(std::set<T1, T2, T3> &a0, std::multiset<T1, T4, T3> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
