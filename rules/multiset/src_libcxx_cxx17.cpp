#include "../common/init.h"
#include "../common/traits.h"
#include <initializer_list>
#include <set>
#include <utility>

template <typename T1, typename T2, typename T3, typename T4>
void f101(
    std::multiset<T1, T2, T3> &a0,
    std::multiset<typename std::multiset<T1, T2, T3>::key_type, T4,
                  typename std::multiset<T1, T2, T3>::allocator_type> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f102(
    std::multiset<T1, T2, T3> &a0,
    std::multiset<typename std::multiset<T1, T2, T3>::key_type, T4,
                  typename std::multiset<T1, T2, T3>::allocator_type> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4>
void f103(std::multiset<T1, T2, T3> &a0,
          std::set<typename std::multiset<T1, T2, T3>::key_type, T4,
                   typename std::multiset<T1, T2, T3>::allocator_type> &a1) {
  return a0.template merge<T4>(a1);
}

template <typename T1, typename T2, typename T3, typename T4>
void f104(std::multiset<T1, T2, T3> &a0,
          std::set<typename std::multiset<T1, T2, T3>::key_type, T4,
                   typename std::multiset<T1, T2, T3>::allocator_type> &&a1) {
  return a0.template merge<T4>(std::move(a1));
}
