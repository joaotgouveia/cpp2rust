#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
void f108(std::set<T1, T2, T3> &a0,
          std::set<typename std::set<T1, T2, T3>::key_type, T4,
                   typename std::set<T1, T2, T3>::allocator_type> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f109(std::set<T1, T2, T3> &a0,
          std::set<typename std::set<T1, T2, T3>::key_type, T4,
                   typename std::set<T1, T2, T3>::allocator_type> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f110(std::set<T1, T2, T3> &a0,
          std::multiset<typename std::set<T1, T2, T3>::key_type, T4,
                        typename std::set<T1, T2, T3>::allocator_type> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f111(std::set<T1, T2, T3> &a0,
          std::multiset<typename std::set<T1, T2, T3>::key_type, T4,
                        typename std::set<T1, T2, T3>::allocator_type> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
