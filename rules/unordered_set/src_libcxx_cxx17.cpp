#include <initializer_list>
#include <unordered_set>
#include <utility>

#include "../common/init.h"
#include "../common/traits.h"

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f98(std::unordered_set<T1, T2, T3, T4> &a0,
         std::unordered_set<
             typename std::unordered_set<T1, T2, T3, T4>::key_type, T5, T6,
             typename std::unordered_set<T1, T2, T3, T4>::allocator_type> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f99(
    std::unordered_set<T1, T2, T3, T4> &a0,
    std::unordered_set<
        typename std::unordered_set<T1, T2, T3, T4>::key_type, T5, T6,
        typename std::unordered_set<T1, T2, T3, T4>::allocator_type> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f100(
    std::unordered_set<T1, T2, T3, T4> &a0,
    std::unordered_multiset<
        typename std::unordered_set<T1, T2, T3, T4>::key_type, T5, T6,
        typename std::unordered_set<T1, T2, T3, T4>::allocator_type> &a1) {
  return a0.template merge<T5, T6>(a1);
}

template <typename T1, typename T2, typename T3, typename T4, typename T5,
          typename T6>
void f101(
    std::unordered_set<T1, T2, T3, T4> &a0,
    std::unordered_multiset<
        typename std::unordered_set<T1, T2, T3, T4>::key_type, T5, T6,
        typename std::unordered_set<T1, T2, T3, T4>::allocator_type> &&a1) {
  return a0.template merge<T5, T6>(std::move(a1));
}
